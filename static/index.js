let fileSim;
let fbManager;

const HIGHLIGHT_COLOUR = "#FF4A1C";
const BUBBLE_COLOUR = "#403F4C";
async function setup() {
  createCanvas(windowWidth, windowHeight);
  fileSim = await getFileSim();
  fbManager = new FileBubbleManager(fileSim);
}

function draw() {
  background("#1B2432");
  fbManager.update();
  fbManager.draw();
}

function windowResized() {
  resizeCanvas(windowWidth, windowHeight);
}

setInterval(async () => {
  fileSim = await getFileSim();
}, 1000);

async function getFileSim() {
  let response = await fetch("/file_connections");
  return JSON.parse(await response.json()).cache;
}

class FileBubbleManager {
  constructor(fileSims) {
    this.fileSims = fileSims;

    this.fileBubbles = {};
    for (const name in fileSims) {
      this.fileBubbles[name] = new FileBubble(name, width / 2, height / 2);
    }
    this.springs = {};
    for (const name in fileSims) {
      const currSprings = [];
      for (const { file_path: otherName } of this.fileSims[name]
        .n_most_similar) {
        const curr = this.fileBubbles[name];
        const other = this.fileBubbles[otherName];
        const restLength = (curr.rad + other.rad) * 6;
        currSprings.push(new Spring(curr, other, restLength));
      }
      this.springs[name] = currSprings;
    }
  }

  update() {
    Object.values(this.fileBubbles).forEach((fb) => {
      this.applyDrag(fb);
      // this.applyGravity(fb);
      this.applySpringForce(fb);
      this.applySpreadForce(fb);
      fb.update();
    });
  }

  draw() {
    this.drawConnections();
    Object.values(this.fileBubbles).forEach((fb) => {
      fb.draw();
    });
  }

  applyGravity(fb) {
    let gravity = createVector(0, 0.5);
    fb.applyForce(gravity);
  }
  applyDrag(fb) {
    let c = 0.01;
    let speed = fb.vel.mag();
    let dragMagnitude = c * speed * speed;
    let drag = fb.vel.copy();
    drag.mult(-1);
    drag.setMag(dragMagnitude);
    fb.applyForce(drag);
  }

  applySpringForce(fb) {
    for (const sp of this.springs[fb.fullName]) {
      sp.otherFB.applyForce(sp.computeForce());
    }
  }

  applySpreadForce(fb) {
    const spreadFactor = 0.01;
    for (const other of Object.values(this.fileBubbles)) {
      const force = p5.Vector.sub(fb.pos, other.pos);
      let distSq = force.magSq();
      if (distSq == 0) {
        continue;
      }
      force.div(distSq); // Inverse distance squared
      force.mult(spreadFactor);
      fb.applyForce(force);
    }
  }

  drawConnections() {
    push();
    stroke("black");
    const highlighted_connections = [];
    for (const [currFile, matchFiles] of Object.entries(this.fileSims)) {
      const curr = this.fileBubbles[currFile];
      if (curr.isHovered) {
        // Save the hovered ones for last
        highlighted_connections.push({
          curr: curr,
          others: matchFiles.n_most_similar,
        });
        continue;
      }
      for (const { file_path } of matchFiles.n_most_similar) {
        const other = this.fileBubbles[file_path];

        line(curr.pos.x, curr.pos.y, other.pos.x, other.pos.y);
      }
    }

    stroke(HIGHLIGHT_COLOUR);
    for (const { curr, others } of highlighted_connections) {
      for (const { file_path } of others) {
        const other = this.fileBubbles[file_path];

        line(curr.pos.x, curr.pos.y, other.pos.x, other.pos.y);
      }
    }

    pop();
  }
}

class FileBubble {
  constructor(fullName, x, y) {
    this.fullName = fullName;
    let splits = fullName.split("/");
    let name = splits[splits.length - 1];
    this.name = name;

    this.pos = createVector(x, y);
    // this.vel = createVector();
    this.vel = p5.Vector.random2D().mult(10);
    this.acc = createVector();
    let rad = textWidth(this.name);
    this.rad = rad * 0.7;
    this.isClicked = false;
    this.isHovered = false;
  }

  update() {
    // Don't apply force if clicked
    const mouse = createVector(mouseX, mouseY);
    const mouseDist = mouse.dist(this.pos);
    if (mouseDist < this.rad) {
      this.isHovered = true;
      if (mouseIsPressed) {
        this.isClicked = true;
        this.vel = createVector(0, 0);
        this.pos = mouse;
      }
    }
    if (!this.isClicked) {
      // Update the pos with physics
      this.vel.add(this.acc);
      this.pos.add(this.vel);
    }
    this.acc.mult(0);

    if (this.pos.x + this.rad > width) {
      this.vel.x *= -1;
      this.pos.x = width - this.rad;
    }

    if (this.pos.x - this.rad < 0) {
      this.vel.x *= -1;
      this.pos.x = 0 + this.rad;
    }

    if (this.pos.y + this.rad > height) {
      this.vel.y *= -1;
      this.pos.y = height - this.rad;
    }

    if (this.pos.y - this.rad < 0) {
      this.vel.y *= -1;
      this.pos.y = 0 + this.rad;
    }
  }

  draw() {
    push();
    fill(BUBBLE_COLOUR);
    if (this.isHovered) {
      stroke(HIGHLIGHT_COLOUR);
    }
    circle(this.pos.x, this.pos.y, this.rad * 2);

    fill("white");
    stroke("black");
    textAlign(CENTER, CENTER);
    text(this.name, this.pos.x, this.pos.y);

    pop();
    this.isClicked = false;
    this.isHovered = false;
  }

  applyForce(force) {
    this.acc.add(force);
  }
}

class Spring {
  constructor(currFB, otherFB, restLength) {
    this.currFB = currFB;
    this.otherFB = otherFB;
    this.restLength = restLength;

    this.length = p5.Vector.sub(currFB.pos, otherFB.pos).mag();
    this.prevLength = this.length;
  }

  computeForce() {
    const k = 0.001;
    const force = p5.Vector.sub(this.currFB.pos, this.otherFB.pos);
    this.length = force.mag();
    const x = this.length - this.restLength;

    const vel = this.length - this.prevLength;
    const damping = 0.01;
    force.setMag(k * x + vel * damping);
    this.prevLength = this.length;
    return force;
  }
}

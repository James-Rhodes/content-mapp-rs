const BUBBLE_COLOUR = "#403F4C";
const HIGHLIGHT_COLOUR = "#FF4A1C";

class FileBubble {
  constructor(fullName, x, y, rad) {
    this.fullName = fullName;
    let splits = fullName.split("/");
    let name = splits[splits.length - 1];
    this.name = name;

    this.pos = createVector(x, y);
    // this.vel = createVector();
    this.vel = p5.Vector.random2D().mult(10);
    this.acc = createVector();
    this.rad = rad;
    this.isClicked = false;
    this.shouldHighlight = false;

    this.pathLife = 0;
  }

  update(bounds) {
    // Don't apply force if clicked
    const mouse = createVector(mouseX, mouseY);
    const mouseDist = mouse.dist(this.pos);
    if (mouseDist < this.rad) {
      this.shouldHighlight = true;
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

    if (this.pos.x + this.rad > bounds.right) {
      this.vel.x *= -1;
      this.pos.x = bounds.right - this.rad;
    }

    if (this.pos.x - this.rad < bounds.left) {
      this.vel.x *= -1;
      this.pos.x = bounds.left + this.rad;
    }

    if (this.pos.y + this.rad > bounds.bottom) {
      this.vel.y *= -1;
      this.pos.y = bounds.bottom - this.rad;
    }

    if (this.pos.y - this.rad < bounds.top) {
      this.vel.y *= -1;
      this.pos.y = bounds.top + this.rad;
    }
  }

  draw() {
    push();
    fill(BUBBLE_COLOUR);
    if (this.shouldHighlight) {
      stroke(HIGHLIGHT_COLOUR);
    }
    circle(this.pos.x, this.pos.y, this.rad * 2);

    fill("white");
    stroke("black");

    if (this.pathLife > 0) {
      this.pathLife -= deltaTime;
      textAlign(CENTER, CENTER);
      text(this.fullName, this.pos.x, this.pos.y - this.rad);
    }

    pop();
    this.isClicked = false;
    this.shouldHighlight = false;
  }

  applyForce(force) {
    this.acc.add(force);
  }

  highlight() {
    this.shouldHighlight = true;
  }

  showPath(numSeconds) {
    this.pathLife = numSeconds * 1000;
  }
}

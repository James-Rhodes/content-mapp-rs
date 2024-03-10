class FileBubbleManager {
  constructor(
    fileSims,
    bounds = { top: 0, bottom: height, left: 0, right: width },
  ) {
    this.fileSims = fileSims;

    this.fileBubbles = {};
    let numFS = Object.keys(fileSims).length;
    let minRad = 10;
    let maxRad = 50;
    for (const name in fileSims) {
      const rad = min(max(map(numFS, 0, 100, maxRad, minRad), minRad), maxRad);
      let x = random(bounds.left + rad, bounds.right - rad);
      let y = random(bounds.top + rad, bounds.bottom - rad);
      this.fileBubbles[name] = new FileBubble(name, x, y, rad);
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

      this.drawLast = [];
    }
  }

  update(bounds) {
    Object.values(this.fileBubbles).forEach((fb) => {
      this.applyDrag(fb);
      // this.applyGravity(fb);
      this.applySpringForce(fb);
      this.applySpreadForce(fb);
      fb.update(bounds);
    });
  }

  draw() {
    this.drawConnections();

    for (const [path, fb] of Object.entries(this.fileBubbles)) {
      if (this.drawLast.includes(path)) {
        continue;
      }
      fb.draw();
    }

    for (const path of this.drawLast) {
      let fb = this.fileBubbles[path];
      fb.draw();
    }
  }

  applyGravity(fb) {
    let gravity = createVector(0, 0.5);
    fb.applyForce(gravity);
  }
  applyDrag(fb) {
    let c = 0.05;
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
    const spreadFactor = 1;
    const maxForce = 10;
    let count = 0;
    let desiredSeparation = fb.rad * 2;
    let sum = createVector();
    for (const other of Object.values(this.fileBubbles)) {
      let d = p5.Vector.dist(fb.pos, other.pos);
      if (fb !== other && d < desiredSeparation) {
        let diff = p5.Vector.sub(fb.pos, other.pos);
        diff.normalize();
        sum.add(diff);
        count++;
      }
    }

    if (count > 0) {
      sum.div(count);
      sum.setMag(spreadFactor);
      let steer = p5.Vector.sub(sum, fb.vel);
      steer.limit(maxForce);
      fb.applyForce(steer);
    }
  }

  drawConnections() {
    push();
    stroke("black");
    const highlighted_connections = [];
    for (const [currFile, matchFiles] of Object.entries(this.fileSims)) {
      const curr = this.fileBubbles[currFile];
      if (curr.shouldHighlight) {
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

  highlightBubble(name) {
    this.fileBubbles[name].highlight();
  }

  drawPaths(path) {
    this.drawLast = [];
    this.drawLast.push(path);

    this.fileBubbles[path].showPath(5);
    for (const sp of this.springs[path]) {
      sp.otherFB.showPath(5);
      this.drawLast.push(sp.otherFB.fullName);
    }
  }
}

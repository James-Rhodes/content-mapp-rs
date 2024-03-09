class PathViewer {
  constructor(paths, fbManager) {
    this.paths = paths.sort();
    this.fbManager = fbManager;

    let maxWidth = 0;
    for (const name of this.paths) {
      const currentWidth = textWidth(truncatePath(name));
      if (currentWidth > maxWidth) {
        maxWidth = currentWidth;
      }
    }
    this.buffer = 20;
    this.fontSize = 14;
    this.fontSpacing = 5;

    this.width = maxWidth + this.buffer;
    this.height = this.fontSize * this.paths.length;
    this.startY = 0;
    this.startX = width - this.width;

    this.show = false;
  }

  draw() {
    if (!this.show) {
      return;
    }

    push();
    fill("#403F4C");
    rect(width - this.width, 0, this.width, height);

    textAlign(LEFT, TOP);
    textSize(this.fontSize);
    this.startX = width - this.width;
    // text(this.name, this.pos.x, this.pos.y);
    for (let [idx, path] of Object.entries(this.paths)) {
      idx = parseInt(idx);
      let truncPath = truncatePath(path);

      let x = this.startX + this.buffer / 2;
      let y = idx * this.fontSize + this.buffer / 2 + this.startY;

      if (idx !== 0) {
        y = y + this.fontSpacing * idx;
      }

      if (this.isHovered() && mouseY < y + this.fontSize && mouseY > y) {
        fill(HIGHLIGHT_COLOUR);
        this.fbManager.highlightBubble(path);
        if (mouseIsPressed) {
          this.fbManager.drawPaths(path);
        }
      } else {
        fill("white");
      }
      text(truncPath, x, y);
    }
    pop();
  }
  isHovered() {
    return mouseX > this.startX && mouseX < width;
  }

  scroll(delta) {
    delta *= -1;
    if (
      this.isHovered() &&
      this.fontSize * this.paths.length > height &&
      this.startY + delta <= 0 &&
      this.startY + delta >=
        -this.height - this.fontSpacing * (this.paths.length - 1)
    ) {
      this.startY += delta;
    }
  }

  toggleShow() {
    this.show = !this.show;
  }
}

function truncatePath(path) {
  let truncateLength = 50;
  if (path.length < truncateLength) {
    return path;
  }
  return path.substr(path.length - truncateLength);
}

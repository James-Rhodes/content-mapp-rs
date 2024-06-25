class Spring {
  constructor(currFB, otherFB, restLength) {
    this.currFB = currFB;
    this.otherFB = otherFB;
    this.restLength = restLength;

    this.length = p5.Vector.sub(currFB.pos, otherFB.pos).mag();
    this.prevLength = this.length;
  }

  computeForce() {
    // Need to figure out how to get the spring force to be nice. Just applying
    // nothing for now
    return createVector();

    // TODO: This doesn't look that good. It should be a soft attraction towards its similar files
    // const k = 0.001;
    // const force = p5.Vector.sub(this.currFB.pos, this.otherFB.pos);
    // this.length = force.mag();
    // const x = this.length - this.restLength;
    //
    // const vel = this.length - this.prevLength;
    // const damping = 0.01;
    // force.setMag(k * x + vel * damping);
    // this.prevLength = this.length;
    // const attractionMultiplier = 0.001;
    // const force = p5.Vector.sub(this.currFB.pos, this.otherFB.pos);
    // const dist = force.mag();
    // if (dist > (this.currFB.rad + this.otherFB.rad) * 20) {
    //   force.normalize();
    //   force.mult(attractionMultiplier * dist);
    //   return force.mult(attractionMultiplier);
    // }
    // return createVector();
  }
}

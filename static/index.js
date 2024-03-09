let fileSim;
let fbManager;
let pathViewer;

async function setup() {
  createCanvas(windowWidth, windowHeight);
  fileSim = await getFileSim();
  fbManager = new FileBubbleManager(fileSim);
  pathViewer = new PathViewer(Object.keys(fileSim), fbManager);
}

function draw() {
  background("#1B2432");
  let bounds;

  if (pathViewer.show) {
    bounds = {
      top: 0,
      bottom: height,
      left: 0,
      right: width - pathViewer.width,
    };
  } else {
    bounds = {
      top: 0,
      bottom: height,
      left: 0,
      right: width,
    };
  }
  fbManager.update(bounds);

  fbManager.draw();
  pathViewer.draw();
}

function windowResized() {
  resizeCanvas(windowWidth, windowHeight);
}

// setInterval(async () => {
//   fileSim = await getFileSim();
// }, 1000);

async function getFileSim() {
  let response = await fetch("/file_connections");
  return JSON.parse(await response.json()).cache;
}

function mouseWheel(event) {
  pathViewer.scroll(event.delta);
}

function keyPressed() {
  // Pressed t
  if (keyCode == 84) {
    pathViewer.toggleShow();
  }
}

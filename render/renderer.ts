class Renderer {
  canvas = document.querySelector("canvas")!;
  context = this.canvas.getContext("2d")!;
  renderParticle(particle: Particle, style="solid") {
    if (style == "glowing"){
      // Could be cool to render the color differently depending on temperature!
      this.context.fillStyle = particle.color;
      this.context.shadowColor = particle.color;
      this.context.shadowBlur = 10;
      this.context.fillRect(particle.x, particle.y, 1, 1);
    }
    if (style == "gas"){
      this.context.beginPath();
      // Need a function to convert the color
      this.context.fillStyle = "rgba(255, 255, 255, 0.5)";
      this.context.arc(particle.x, particle.y, 3, 0, 2 * Math.PI);
      this.context.fill();
    }
    else {
      this.context.fillStyle = particle.color;
      this.context.fillRect(particle.x, particle.y, 1, 1);
    }
  }
  clear() {
    this.context.clearRect(0, 0, this.canvas.width, this.canvas.height);
  }
}

class EventHandler {
  drawingMode = false;
  drawMode() {
    renderer.canvas.addEventListener('mousedown', () => {
      this.drawingMode = true;
    });
    renderer.canvas.addEventListener('mousemove', (e) => {
      if (this.drawingMode) {
      }
    });
    renderer.canvas.addEventListener('mouseup', () => {
      this.drawingMode = false;
    });
  }
}

var renderer = new Renderer();
var events = new EventHandler();
events.drawMode();

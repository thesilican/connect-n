self.addEventListener("message", (evt) => {
  console.log("AI Request:", evt.data);
  setTimeout(() => {
    self.postMessage(Math.floor(Math.random() * 7));
  }, 1000);
});

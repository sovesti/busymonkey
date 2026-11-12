let timer = null;

function stop() {
  clearInterval(timer);
  timer = null;
  chrome.action.setBadgeText({ text: "" });
}

stop();

chrome.action.onClicked.addListener((tab) => {
  if (timer !== null) {
    stop();
    return;
  }

  const windowId = tab.windowId;
  timer = setInterval(async () => {
    try {
      const tabs = await chrome.tabs.query({ windowId });
      if (timer === null) return;
      if (tabs.length === 0) {
        stop();
        return;
      }

      tabs.sort((a, b) => a.index - b.index);
      const current = tabs.findIndex((tab) => tab.active);
      const next = tabs[(current + 1) % tabs.length];
      await chrome.tabs.update(next.id, { active: true });
    } catch {
      stop();
    }
  }, 5000);

  chrome.action.setBadgeText({ text: "ON" });
});

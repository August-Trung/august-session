const PORT = 18942;
const BASE_URL = `http://127.0.0.1:${PORT}`;

async function pollForCapture() {
  try {
    const response = await fetch(`${BASE_URL}/wait-for-capture`);
    if (response.status === 200) {
      const data = await response.json();
      if (data.capture === true) {
        // Query all tabs in all windows
        chrome.tabs.query({}, async (tabs) => {
          // Filter to only include valid HTTP/HTTPS URLs (exclude internal chrome:// pages if desired, but let's keep them or filter them)
          const tabData = tabs
            .filter(tab => tab.url && (tab.url.startsWith("http://") || tab.url.startsWith("https://") || tab.url.startsWith("file://")))
            .map(tab => ({
              windowId: tab.windowId,
              index: tab.index,
              url: tab.url,
              title: tab.title || "",
              active: tab.active
            }));

          // Send them back to Tauri local server
          try {
            await fetch(`${BASE_URL}/submit-tabs`, {
              method: 'POST',
              headers: {
                'Content-Type': 'application/json'
              },
              body: JSON.stringify(tabData)
            });
          } catch (e) {
            console.error("Failed to submit tabs:", e);
          }
        });
      }
    }
  } catch (err) {
    // If the server is offline, wait 5 seconds before trying again
    console.log("August Session server offline, retrying in 5s...");
    await new Promise(resolve => setTimeout(resolve, 5000));
  }
  
  // Immediately poll again
  pollForCapture();
}

// Start polling
pollForCapture();

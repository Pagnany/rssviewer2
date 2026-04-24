import { invoke } from "@tauri-apps/api/core";
import { fetch } from '@tauri-apps/plugin-http';

let rssFeedD1: HTMLElement | null;

window.addEventListener("DOMContentLoaded", () => {
  rssFeedD1 = document.querySelector("#rss-feed");

  // Refresh Button Click
  document.querySelector("#rss-refresh")!.addEventListener("click", async (e) => {
    e.preventDefault();

    // Example of using the fetch API to make an HTTP request
    const response = await fetch('https://www.google.com/', {
      method: 'GET',
    });
    rssFeedD1!.textContent = response.status + " " + response.statusText;

    invoke('test_http', { url: 'https://www.pagnany.de/' })
      .catch((error) => console.error(error));
  });

  // Go to Top Button Click
  document.getElementById("go-top")!.addEventListener("click", () => {
    window.scrollTo(0, 0);
  });
});


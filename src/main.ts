import { invoke } from "@tauri-apps/api/core";

let rssFeedD1: HTMLElement | null;

window.addEventListener("DOMContentLoaded", () => {
  rssFeedD1 = document.querySelector("#rss-feed");

  // Refresh Button Click
  document.querySelector("#rss-refresh")!.addEventListener("click", async (e) => {
    e.preventDefault();

    invoke('get_rssfeed_channels').then((result: any) => {
      rssFeedD1!.textContent = result;
    })
      .catch((error) => console.error(error));
  });

  // Go to Top Button Click
  document.getElementById("go-top")!.addEventListener("click", () => {
    window.scrollTo(0, 0);
  });
});


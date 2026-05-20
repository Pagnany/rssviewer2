import { invoke } from "@tauri-apps/api/core";

let rssFeedD1: HTMLElement | null;
let rssErrorD1: HTMLElement | null;

window.addEventListener("DOMContentLoaded", () => {
  rssFeedD1 = document.querySelector("#rss-feed");
  rssErrorD1 = document.querySelector("#rss-error");

  // Refresh Button Click
  document.querySelector("#rss-refresh")!.addEventListener("click", async (e) => {
    e.preventDefault();

    invoke('get_rssfeeds').then((message: any) => {
      let rssDate = "";
      message.rssfeeditems.forEach((item: any) => {
        rssDate += "<article>";
        rssDate += "<p>" + item.feed_name + "</p>";
        rssDate += "<h4>" + item.header + "</h4>";
        rssDate += "<p>" + item.description + "</p>";
        rssDate += "<img src='" + item.image + "'/>" + "<br />";

        rssDate +=
          '<a href="' +
          item.url +
          ' " target="_blank">' +
          "Link" +
          "</a>" +
          "<br />";

        rssDate += item.date + "<br />";
        rssDate += "</article>";
        rssDate += "<br/>";
      });

      rssFeedD1!.innerHTML = rssDate;

      let rssError = "";
      message.errors.forEach((error: any) => {
        rssError += "<p>" + error + "</p>";
      });
      rssErrorD1!.innerHTML = rssError;
    })
      .catch((error) => {
        rssErrorD1!.textContent = error;
        console.error(error)
      });
  });

  // Go to Top Button Click
  document.getElementById("go-top")!.addEventListener("click", () => {
    window.scrollTo(0, 0);
  });
});


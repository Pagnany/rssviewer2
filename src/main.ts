import { invoke } from "@tauri-apps/api/core";

let rssFeedD1: HTMLElement | null;

window.addEventListener("DOMContentLoaded", () => {
  rssFeedD1 = document.querySelector("#rss-feed");

  // Refresh Button Click
  document.querySelector("#rss-refresh")!.addEventListener("click", async (e) => {
    e.preventDefault();

    invoke('get_rssfeeds').then((message: any) => {
      rssFeedD1!.textContent = message;
      let rssDate = "";
      message.forEach((item: any) => {
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
    })
      .catch((error) => {
        rssFeedD1!.textContent = error;
        console.error(error)
      });
  });

  // Go to Top Button Click
  document.getElementById("go-top")!.addEventListener("click", () => {
    window.scrollTo(0, 0);
  });
});


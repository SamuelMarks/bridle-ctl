const fs = require("fs");
const cheerio = require("cheerio");
const $ = cheerio.load(
  fs.readFileSync("docs/components/AppInputComponent.html"),
);
$(".io-description").each((i, el) => {
  const text = $(el).text().trim();
  if (text === "") {
    console.log($(el).parent().prev().text());
  }
});

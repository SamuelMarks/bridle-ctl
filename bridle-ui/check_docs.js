const fs = require("fs");
const cheerio = require("cheerio");
const $ = cheerio.load(fs.readFileSync("docs/coverage.html"));
$("table tbody tr").each((i, el) => {
  const file = $(el).find("td").eq(0).text().trim();
  const type = $(el).find("td").eq(1).text().trim();
  const name = $(el).find("td").eq(2).text().trim();
  const percent = parseInt($(el).find(".coverage-percent").text());
  if (percent < 100) {
    console.log(file, "|", type, "|", name, "|", percent + "%");
  }
});

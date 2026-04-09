const fs = require('fs');
const cheerio = require('cheerio');
const $ = cheerio.load(fs.readFileSync('docs/directives/AppTableColumnDirective.html'));
$('.io-description').each((i, el) => {
  if ($(el).text().trim() === '') console.log('Empty desc for:', $(el).parent().prev().text());
});
console.log('Total io-description:', $('.io-description').length);

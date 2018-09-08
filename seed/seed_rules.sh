echo "Seed rules"
curl -X POST -H "Content-type: application/json" --data "{\
 \"xpath\": \"//a[contains(@class, 'productGTMClick')]/@href\",\
 \"regexp_rule\": \"0+(\\\\d*)\\\\.html\",\
 \"regexp_flags\": \"\"\
 }" http://picard.espitallier.fr:7777/api/picard/product_rules
echo


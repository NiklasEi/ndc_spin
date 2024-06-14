#import "invoice.typ": *

#let input = if sys.inputs.keys().contains("input") {json.decode(sys.inputs.input);} else {json("sample.json");}

#show: invoice.with(
  language: "de", // or "en"
  banner-image: image(input.banner),
  invoice-id: input.id,
  // // Uncomment this to create a cancellation invoice
  // cancellation-id: "2024-03-24t210835",
  issuing-date: input.issuingDate,
  delivery-date: input.deliveryDate,
  due-date: input.dueDate,
  biller: input.biller,
  hourly-rate: 100, // For any items with `dur-min` but no `price`
  recipient: input.recipient,
  items: input.items,
)
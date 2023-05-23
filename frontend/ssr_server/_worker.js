import init, { render } from './ssr.js';


// async function run() {
//   await init();
//   const result =  render();
//   document.body.innerHTML = result;
// }
//
// run();

addEventListener("fetch", (event) => {
  event.respondWith(handleSsr(event.request));
});


export const handleSsr = async (_) => {
  console.log("sentinel1");
  // todo: 設計的に、wasm内部に全部のコードをまとめるのはinfraをつっこんでるようなもの
  // なのでHTMLをtsで取得して、それをwasmに渡してSSRした方が自然な気もする
  await init();
  console.log("sentinel2");
  const result = await render();
  console.log("sentinel3");
  console.log({ result })
  const html = 'aaa';

  console.log("aaaaaaaaaaaaaaaa");
  return new Response(html, {
    status: 200,
    headers: {
      'content-type': 'text/plain;charset=UTF-8'
    }
  });
};

// export const onRequest = [handleSsr];

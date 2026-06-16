import adapter from '@sveltejs/adapter-static';
 
/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    adapter: adapter({
      fallback: 'index.html'
    }),
    prerender: {
      handleHttpError: ({ path, referrer, message }) => {
        // ignore missing favicon and other static assets
        if (path.startsWith('/favicon') || path.startsWith('/_app')) return;
        throw new Error(message);
      }
    }
  }
};
 
export default config;

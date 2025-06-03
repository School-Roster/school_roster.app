import type { PageLoad } from './$types';

import "$styles/form/desing.scss";

export const prerender = true;

export const load: PageLoad = ({ params }) => {
  console.log(params);
  return {
    page: params.windowName
  };
};

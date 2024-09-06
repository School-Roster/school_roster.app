import type { PageLoad } from './$types';

import "$styles/form/desing.scss";

export const load: PageLoad = ({ params }) => {
  console.log(params);
  return {
    page: params.windowName
  };
};

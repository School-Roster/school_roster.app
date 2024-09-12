import type { PageLoad } from './$types';

/* TODO: Hay un error en este archivo, por alguna razon despues
          de recompilar el proyecto al abrir una pagina se abre la misma. */
import "$styles/form/desing.scss";

export const load: PageLoad = ({ params }) => {
  console.log(params);
  return {
    page: params.windowName
  };
};

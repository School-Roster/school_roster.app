/**
  * Funcion para detectar click fuera de un elemento en Svelte
  * @param {HTMLElement} node - Elemento al que se le detectara el click fuera
  */
export function clickOutside(node: HTMLElement) {
  
  const handleClick = (event: MouseEvent) => {
    if (node && !node.contains(event.target as Node) && !event.defaultPrevented) {
      node.dispatchEvent(
        new CustomEvent('click_outside', { detail: node })
      );
    }
  };

  document.addEventListener('click', handleClick, true);
  
  return {
    destroy() {
      document.removeEventListener('click', handleClick, true);
    }
  };
}

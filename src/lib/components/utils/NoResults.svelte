<div class="container--without-results">
  <img src="/icons/close.svg" alt="error" />
  <h1>No se encontraron resultados</h1>
  <p>Parece que aún no hay datos. ¡Es un buen momento para comenzar a añadir algo nuevo!</p>
  <p>¿Crees que esto es un error? <a href="/report-issue" class="link">Infórmanos</a>.</p>
</div>

<style lang="scss">
  .container--without-results {
    display: flex;
    flex-direction: column;
    justify-items: center;
    justify-content: center;
    align-items: center;
    padding: 16px;
    text-align: center;
    flex-direction: column;
    gap: 6px;

    img {
      margin-top: 32px;
      max-width: 100%;
      max-height: 100px;
      object-fit: contain;
      border-radius: 8px;
    }

    h1 {
      font-family: "Inria Sans";
      color: #191724;
      font-size: 2.25rem;
      font-weight: bold;
      font-style: bold;
      margin: 16px 0;
    }

    p {
      margin-top: 0;
      font-size: 1rem;
    }
  }
</style>

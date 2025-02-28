<script lang="ts">
    let username = "";
    let password = "";
    let error = "";
  
    async function handleSubmit(event: Event) {
      event.preventDefault();
      
      const res = await fetch("/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username, password })
      });
  
      const data = await res.json();
      if (!res.ok) {
        error = data.error;
      } else {
        window.location.href = "/dashboard"; 
      }
    }
  </script>

<div class="container">
  <h2> <img class="logo-icon" src="/icons/logosr-fr.png" alt="Icon" />School Rooster</h2>
<!--   <h3>Administrador y generador de horarios optimizado.</h3>
 -->  <form on:submit={handleSubmit}>
    <label for="username">Usuario</label>
    <input id="username" type="text" bind:value={username} required />

    <label for="password">Contraseña</label>
    <input id="password" type="password" bind:value={password} required />

    {#if error}
      <p class="error">{error}</p>
    {/if}

    <button type="submit">Ingresar</button>

    <button class="google-btn" type="submit"> <img class="google-icon" src="/icons/google-icon.svg" alt="Icon" />Sing in with Google</button>


  </form>
</div>

  
<style lang="scss">
    .container {
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      height: 550px;
      width: 500px;  
      padding: 30px;     
      background: #fff;
      border-radius: 10px;
      box-shadow: 0px 4px 15px rgba(0, 0, 0, 0.2); 
    }
      h2 {
        text-align: center;
        font-size: 40px;
        font-weight: bold;
        color: #5e81ac;
        margin-right: 80px;
      }
    /*   h3 {
        text-align: center;
        font-size: 18px;
        color: #87a9d4;
      } */
      
      form {
        display: flex;
        flex-direction: column;

      
      label {
        margin: 10px 0 5px;
        margin-left: 70px;
        font-weight: bold;

      }
  
      input {
        padding: 10px;
        width: 350px;
        border-radius: 10px;
        border: 1px solid #ccc;
        border-radius: 5px;
        margin-left: 70px;
            
        }
  
      .error {
        color: red;
        text-align: center;
        margin-top: 10px;
      }
  
      button {
        margin-top: 15px;
        padding: 12px;
        background: #5e81ac;
        height: 40px;

        color: #fff;
        border: none;
        border-radius: 10px;
        cursor: pointer;
        font-size: 16px;
        width: 373px;
        margin-left: 70px;
        
  
        &:hover {
          background: #7f98b6;
        }
        
      }
     .google-btn {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 373px;
        height: 40px;
        padding: 12px;
        background: #ffffff;
        color: #333;
        font-size: 15px;
        font-weight: bold;
        border: 2px solid #000000;
        border-radius: 10px;
        cursor: pointer;
        transition: background 0.3s ease, color 0.3s ease, transform 0.1s ease-in-out;

        &:hover {
          background: #ffffff;
          color: #000000;
        }

        &:active {
          transform: scale(0.95);
        } 
      .google-icon {
          width: 35px;   
          height: 35px;
          margin-right: 12px;
        }

        img {
          width: 20px;
          height: 20px;
          margin-right: 10px;
        }
      


    }
        .message {
      text-align: center;
      margin-top: 15px;
      font-size: 16px;
      color: #333; 

      a {
        color: #ff1e00; 
        text-decoration: none;
        font-weight: bold;

        &:hover {
          color: #0056b3;
          text-decoration: underline;
        }
      }
    }

    
      }
      .logo-icon {
      width: 100px;  
      height: auto;  
      margin-left: 40px;
    }


      body {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100vh; 
        background: #f0f2f5;
      }

  
</style>

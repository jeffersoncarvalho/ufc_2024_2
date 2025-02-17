let id_global = 5
getPokemonFetchAsyncAwait(id_global)
imprimirId(id_global);

function imprimirId(id) {
    let hId = document.getElementById("id-id");
    hId.innerHTML = "ID: " + id
}

function pokemon_menos_1() {
    
    id_global = id_global - 1
    if (id_global <= 0) {
        id_global = 1
    }
    imprimirId(id_global)
    getPokemonFetchAsyncAwait(id_global)
}

function pokemon_mais_1() {
    
    id_global = id_global + 1
    if(id_global >= 11){
        id_global = 10
    }
    imprimirId(id_global)
    getPokemonFetchAsyncAwait(id_global)
}

function getPokemonAxios() {
  axios
    .get("https://pokeapi.co/api/v2/pokemon/3")
    .then((response) => {
      //console.log(response.data.name)
      let nome = document.getElementById("nome-id");
      nome.innerHTML = response.data.name;

      let imagem = document.getElementById("pokemon-id");
      imagem.src = response.data.sprites.front_default;

      let imagemSvg = document.getElementById("pokemon-svg-id");
      imagemSvg.src = response.data.sprites.other.dream_world.front_default;
    })
    .catch(() => {
      console.log("error");
    });
}
function getPokemonFetch(id) {
  //fetch(`https://pokeapi.co/api/v2/pokemon/${id}`)
  fetch("https://pokeapi.co/api/v2/pokemon/" + id)
    .then((response) => {
      return response.json();
    })
    .then((json) => {
      let nome = document.getElementById("nome-id");
      nome.innerHTML = json.name;

      let imagem = document.getElementById("pokemon-id");
      imagem.src = json.sprites.front_default;

      let imagemSvg = document.getElementById("pokemon-svg-id");
      imagemSvg.src = json.sprites.other.dream_world.front_default;
    })
    .catch((error) => {
      console.log(error);
    });
}

async function getPokemonAxiosAsyncAwait(id) {
  let response = await axios.get(`https://pokeapi.co/api/v2/pokemon/${id}`);

  let nome = document.getElementById("nome-id");
  nome.innerHTML = response.data.name;

  let imagem = document.getElementById("pokemon-id");
  imagem.src = response.data.sprites.front_default;

  let imagemSvg = document.getElementById("pokemon-svg-id");
  imagemSvg.src = response.data.sprites.other.dream_world.front_default;
}

async function getPokemonFetchAsyncAwait(id) {
  let response = await fetch("https://pokeapi.co/api/v2/pokemon/" + id);
  let json = await response.json();

  let nome = document.getElementById("nome-id");
  nome.innerHTML = json.name;

  let imagem = document.getElementById("pokemon-id");
  imagem.src = json.sprites.front_default;

  let imagemSvg = document.getElementById("pokemon-svg-id");
  imagemSvg.src = json.sprites.other.dream_world.front_default;
}

function eventoBotao() {
  let numeroInput = document.getElementById("numero-id");
  //getPokemonFetch(numeroInput.value);
  //getPokemonAxiosAsyncAwait(numeroInput.value);
  getPokemonFetchAsyncAwait(numeroInput.value);
}

async function asd(){
//await fetch("http://localhost:8080/probacska",{method:"get",credentials:"include",mode:"cors"}).then(async (i)=>console.log(await i.json()))
await fetch("http://localhost:8080/session",{method:"GET",credentials:"include",}).then( async (i)=>console.log(await i.json()))
await fetch("http://localhost:8080/anticsrf",{method:"GET",credentials:"include",}).then( async (i)=>console.log(await i.json()))
}
asd()

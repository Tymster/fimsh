import React, { useContext, useEffect, useState } from "react";
import { Triangle } from "react-feather"
const Home = () => {
  const [id, setid] = useState(0);
  const [name, setname] = useState("");
  const [top, settop] = useState([])
  useEffect(() => {
    fetch("http://127.0.0.1:8080/new").then(
      res => res.json()
    ).then(data => {
      console.log(data)
      setid(data.id);
      setname(data.name)
    })
  }, [])
  async function handleVote(increase: number) {
    fetch(`http://127.0.0.1:8080/update/${id}`, {
      method: "POST",
      body: JSON.stringify({
        "value": increase
      })
    }).then(
      res => res.json()
    ).then(
      data => {
        console.log(data)
        setname(data.name)
        setid(data.id)
      }
    ).catch(err => console.log(err))
  }
  return (
    <div className={`bg-[url('../assets/background.png')]  bg-cover text-white text-2xl w-screen h-screen flex items-center justify-center`}>

      <div className="flex flex-row w-96">
        <h1>{name}</h1>
        {
          top.map((fish) => (
            <div key={Math.random()}>
              {fish}
            </div>
          ))
        }
        <div>
          <Triangle className=" cursor-pointer" onClick={() => handleVote(5)} />
          <Triangle className=" rotate-180 cursor-pointer" onClick={() => handleVote(-5)} />
        </div>
        <div className="w-64" >
          fart
          <img src={`http://127.0.0.1:8080/cdn/${id}`} alt="" />
        </div>
      </div>

    </div >
  );
};

export default Home;
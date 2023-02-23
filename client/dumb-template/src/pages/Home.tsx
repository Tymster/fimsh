import React, { useContext, useEffect, useState } from "react";
import { Triangle } from "react-feather"
const Home = () => {
  const [id, setid] = useState(0);
  function handleVote(increase: number) {
    //do some stuff
    console.log(`Send something with id of ${increase} to ${id}`)
  }
  return (
    <div className="bg-[url('../assets/background.png')] bg-cover text-white text-2xl w-screen h-screen flex items-center justify-center">

      <div className="flex flex-row w-96">
        <div>
          <Triangle className=" cursor-pointer" onClick={() => handleVote(1)} />
          <Triangle className=" rotate-180 cursor-pointer" onClick={() => handleVote(-1)} />
        </div>
        <div className="w-64 bg-red-50">
          fart
        </div>
      </div>

    </div>
  );
};

export default Home;
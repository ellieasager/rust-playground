import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";


const allToppings = [
  { name: "Golden Corn", checked: false },
  { name: "Paneer", checked: false },
  { name: "Tomato", checked: false },
  { name: "Mushroom", checked: false },
  { name: "Onion", checked: false },
  { name: "Black Olives", checked: false },
]

export const Checkbox = ({ isChecked, label, checkHandler, index }) => {
  console.log({ isChecked })
  return (
    <div>
      <input
        type="checkbox"
        id={`checkbox-${index}`}
        checked={isChecked}
        onChange={checkHandler}
      />
      <label htmlFor={`checkbox-${index}`}>{label}</label>
    </div>
  )
}

function App() {
  const [checkMsg, setCheckMsg, ] = useState("");
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  const [toppings, setToppings] = useState(allToppings)

  const updateCheckStatus = index => {
    setToppings(
      toppings.map((topping, currentIndex) =>
        currentIndex === index
          ? { ...topping, checked: !topping.checked }
          : topping
      )
    )
  }

  const selectAll = () => {
    setToppings(toppings.map(topping => ({ ...topping, checked: true })))
  }
  const unSelectAll = () => {
    setToppings(toppings.map(topping => ({ ...topping, checked: false })))
  }

  async function check() {
    setCheckMsg(await invoke("check"));
  }

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="App">

      <p>
        <button onClick={selectAll}>Select All</button>
        <button onClick={unSelectAll}>Unselect All</button>
      </p>

      <div className="card">
      {toppings.map((topping, index) => (
        
        <Checkbox
          key={topping.name}
          isChecked={topping.checked}
          checkHandler={() => updateCheckStatus(index)}
          label={topping.name}
          index={index}
        />
        
      ))}
      </div>
      <div>The all checked values are {toppings.filter(topping => topping.checked).map(topping => topping.name).join(" , ")}</div>

  

      <p>{greetMsg}</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <p>{greetMsg}</p>


      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          check();
        }}
      >
        <button type="submit">Check</button>
      </form>

      <p>{checkMsg}</p>

    </div>
  );
}

export default App;

import { useEffect, useState } from "react"
import CircularTile from "./CircularTile"

const LoadingAnimation = ({className, isLoading}: {className?: string; isLoading: boolean}) => {
  const [rotations, setRotations] = useState([0, 0, 0, 0]);

  const getRandRotation = () => {
    const rand_index = Math.floor(Math.random() * 4)
    const rotation = [0, 90, 180, 270][rand_index]
    return rotation;
  }

  useEffect(() => {
    const interval = setInterval(() => {
      if(isLoading) {
        setRotations([getRandRotation(),getRandRotation(),getRandRotation(),getRandRotation()]);
      }
    }, 500);

    if(!isLoading) {
      clearInterval(interval)
    }

    return () => clearInterval(interval);
  }, [rotations, isLoading]);

  return (
    <div className={`grid grid-cols-2 gap-1 w-10 h-10 ${className}`}>
      {rotations.map((rotation, i) => {
        return (
          <CircularTile key={i} className="!fill-slate-600 duration-500 ease-in-out" rotation={rotation}/>
        )
      })}
    </div>
  )
}

export default LoadingAnimation
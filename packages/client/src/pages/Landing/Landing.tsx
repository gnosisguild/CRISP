import React, { Fragment } from "react";
import HeroSection from "./components/Hero";
import DailyPollSection from "./components/DailyPoll";
import PastPollSection from "./components/PastPoll";

const Landing: React.FC = () => {
  return (
    <Fragment>
      <HeroSection />
      <DailyPollSection />
      <PastPollSection />
    </Fragment>
  );
};

export default Landing;

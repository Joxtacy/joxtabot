const timeoutReasons = [
  "Because! That's why! KEKW",
  "You are just uselessly spending masks",
  "Stop hitting yourself. Stop hitting yourself.",
  "You are just wasting your time",
];

export const getRandomTimeoutReason = () => {
  return timeoutReasons[Math.floor(Math.random() * timeoutReasons.length)];
};

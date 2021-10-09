const timeoutReasons = [
    "Because! That's why! KEKW",
    "You are just uselessly spending masks",
];

export const getRandomTimeoutReason = () => {
    return timeoutReasons[Math.floor(Math.random() * timeoutReasons.length)];
};

export const opcodeHandlers = (socket) => {
    return {
        op10Handler: (sequenceNumber: number, heartbeatInterval: number) => {
            setTimeout(() => {
                const data = {
                    op: 1,
                    d: sequenceNumber,
                };
                socket.send(JSON.stringify(data));
                setInterval(() => {
                    const data = {
                        op: 1,
                        d: sequenceNumber,
                    };
                    socket.send(JSON.stringify(data));
                }, heartbeatInterval);
            }, Math.random() * heartbeatInterval);
        },
    };
};

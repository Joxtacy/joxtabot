import type { FC } from "hono/jsx";

const Main: FC = () => {
	return (
		<html lang="en">
			<head>
				<title>Chatter</title>
				<meta charset="UTF-8" />
				<meta name="viewport" content="width=device-width, initial-scale=1" />
				<script
					src="https://unpkg.com/htmx.org@1.9.10"
					integrity="sha384-D1Kt99CQMDuVetoL1lrYwg5t+9QdHe7NLX/SoJYkXDFfX37iInKRy5xLSi8nO7UC"
					crossorigin="anonymous"
				/>
				{/* Server Sent Events plugin */}
				<script src="https://unpkg.com/htmx.org/dist/ext/sse.js" />
				{/* _hyperscript */}
				<script src="https://unpkg.com/hyperscript.org@0.9.12" />

				<link rel="preconnect" href="https://fonts.googleapis.com" />
				<link
					rel="preconnect"
					href="https://fonts.gstatic.com"
					crossOrigin="anonymous"
				/>
				<link
					href="https://fonts.googleapis.com/css2?family=Inter:wght@100..900&family=Roboto&display=swap"
					rel="stylesheet"
				/>

				<link href="public/style.css" rel="stylesheet" />
			</head>
			<body>
				<div
					class="chat-container"
					hx-ext="sse,remove-me"
					sse-connect="/chat"
					sse-swap="chat"
					hx-swap="beforeend"
				/>
			</body>
		</html>
	);
};

export default Main;

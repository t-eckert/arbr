import { QueryClient, QueryClientProvider } from "react-query"

import { Doc, Nav } from "./sections"

const App = () => {
  return (
    <QueryClientProvider client={new QueryClient()}>
      <div className="w-screen h-screen overflow-hidden flex flex-row">
        <Nav />
        <Doc />
      </div>
    </QueryClientProvider>
  )
}

export default App

import { useQuery, UseQueryResult } from "react-query";
import { Dir } from "../interfaces"

const useDir = (dir: string): UseQueryResult<Dir> => {
	return useQuery(`dir-${dir}`, async (): Promise<Dir> => {
		const response = await fetch(`http://localhost:8000/api/dir${dir}`)
		if (!response.ok) {
			throw new Error(response.statusText)
		}

		const data = await response.json()
		return data
	})
}

export default useDir
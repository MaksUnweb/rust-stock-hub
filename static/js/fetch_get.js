//Basic fetch api get request to specified url

export async function fetch_get(url) {

  try{
  
    const response = await fetch(`/${url}`)

    if(!response.ok) {
      throw new Error("Error sending the request, please try again later!");
    }
    const data = await response.json();

    return data;

  }catch(err) {
    throw err;
  }
}

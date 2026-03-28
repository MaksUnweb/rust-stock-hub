//This is where the fetch api post request is located. 
  //This code is a template, it connects to other files, 
  //and the function is called where needed.



export async function fetch_data(url, data) {
  try{
    const response = await fetch(`/${url}`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(data)
    });

    if (!response.ok){
      throw new Error("Error sending the request, please try again later!");
    }
    
    return await response.json();

  } catch (err) {
    throw err;
  }
}

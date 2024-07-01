// const uploadFile = async (telegramURL: string): Promise<string> => {
//   const resp = await axios.post(
//     config.imgUploadURL,
//     {
//       url: telegramURL,
//     },
//     {
//       headers: {
//         "Content-Type": "application/json",
//       },
//     }
//   );

//   if (resp.status != 200) {
//     throw new Error("Error uploading file.");
//   }

//   const data: { filename: string } = await resp.data;

//   return `${config.imgUploadURL}/${data.filename}`;
// };

// export { uploadFile };


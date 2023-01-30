// Client send file upload
// FileName
// FileSize
// Signal end write

// Server check if file exist
// If exist then send Err
// Server check if file size is correct
// If not then send Err

// If Ok then save fileName, fileSize in hashmap with ID as key and send ID to client

// Client send file upload
// 1. Send ID
// 2. Stream file

// Server receive file
// 1. Receive ID
// 2. Receive file

// server remove ID from hashmap

// In short:
// Client send file upload request including file name and size
// Server check if file exist and if size is correct
// If Ok then server send ID to client
// Client send file upload request including ID
// Server receive file and save it removing ID from hashmap

// ------------------------------

// Client send file download
// FileName
// Signal end write

// Server check if file exist
// If not then send Err
// if Ok then send file size

// Client and server start file transfer stream over TCP
// after file transfer is done client and server close connection
// client save file to disk .
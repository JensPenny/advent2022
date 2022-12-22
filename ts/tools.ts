import * as fs from 'fs';

export class FileReader {
    /**
     * 
     * @param location the file to open
     */
    read(location: string): Promise < string > {
        return new Promise((resolve, reject) => {
            fs.readFile(location, 'utf8', (err, data) => {
                if (err) {
                    reject(err)
                } else {
                    resolve(data)
                }
            });
        });
    }

    asStringList(data: string, separator: string): Array < string > {
        return data.split(separator)
    }
}
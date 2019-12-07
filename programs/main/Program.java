package main;
// ~1s
// runtime: missing file -> Warning
// error message is values are ignored.

// data is invalid -> ignored, Warning
// 	-> runtime error
// java.lang.NullPointerException

import java.io.IOException;
import java.io.FileReader;

import java.io.BufferedReader;
public class Program {
	public static void main(String args[]){
		//String x = null;
		//System.out.println(x.toUpperCase());
		for(int x=0;x<1000;x++){
			try {
				String file = "input.txt";
				BufferedReader reader = new BufferedReader(new FileReader(file));
				String currentLine = reader.readLine();
				int sum = 0;
				while (currentLine != null) {
					System.out.println(currentLine);
					currentLine = reader.readLine();
					if(currentLine =="" || currentLine==null){
						continue;
					}
					try {
						sum += Integer.parseInt(currentLine);
						System.out.println("SUM:" + currentLine + " -> " + sum);
					}catch(NumberFormatException e){
						System.out.println("INVALID:" + currentLine);
					}
				}

				reader.close();
				System.out.println("sum:"+sum);


			}catch(IOException e){
				System.out.println("Whoops!");

			}
		}

	}
}




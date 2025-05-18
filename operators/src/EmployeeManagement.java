import java.io.*;
import java.util.Scanner;

public class EmployeeManagement {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        String fileName = "employees.txt";
        int choice;

        do {
            System.out.println("\n1. Add an Employee");
            System.out.println("2. Display All");
            System.out.println("3. Exit");
            System.out.print("Enter your choice: ");
            choice = scanner.nextInt();
            scanner.nextLine(); // Consume newline

            switch (choice) {
                case 1:
                    try (FileWriter writer = new FileWriter(fileName, true)) {
                        System.out.print("Enter Employee ID: ");
                        String empId = scanner.nextLine();
                        System.out.print("Enter Employee Name: ");
                        String name = scanner.nextLine();
                        System.out.print("Enter Designation: ");
                        String designation = scanner.nextLine();
                        System.out.print("Enter Salary: ");
                        double salary = scanner.nextDouble();
                        scanner.nextLine(); // Consume newline

                        writer.write("Employee ID: " + empId + ", Name: " + name +", Designation: " + designation + ", Salary: " + salary + "\n");
                        System.out.println("Employee details saved successfully!");
                    } catch (IOException e) {
                        System.out.println("Error writing to file: " + e.getMessage());
                    }
                    break;

                case 2:
                    try (BufferedReader reader = new BufferedReader(new FileReader(fileName))) {
                        String line;
                        System.out.println("\nEmployee Details:");
                        while ((line = reader.readLine()) != null) {
                            System.out.println(line);
                        }
                    } catch (IOException e) {
                        System.out.println("Error reading from file: " + e.getMessage());
                    }
                    break;

                case 3:
                    System.out.println("Exiting the program. Goodbye!");
                    break;

                default:
                    System.out.println("Invalid choice! Please try again.");
            }
        } while (choice != 3);

        scanner.close();
    }
}

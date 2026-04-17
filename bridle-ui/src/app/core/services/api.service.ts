import { Injectable, inject } from '@angular/core';
import { HttpClient, HttpErrorResponse } from '@angular/common/http';
import { Observable, throwError } from 'rxjs';
import { catchError, retry } from 'rxjs/operators';

/**
 * Generic API Service for making HTTP requests.
 */
@Injectable({
  providedIn: 'root',
})
export class ApiService {
  /** HTTP Client instance */
  private http = inject(HttpClient);

  /** Base API URL */
  private baseUrl = '/api';

  /**
   * Performs a GET request.
   * @param path The API endpoint path
   * @returns Observable of the response
   */
  get<T>(path: string): Observable<T> {
    return this.http
      .get<T>(`${this.baseUrl}${path}`)
      .pipe(retry(2), catchError(this.handleError));
  }

  /**
   * Performs a POST request.
   * @param path The API endpoint path
   * @param body The request body payload
   * @returns Observable of the response
   */
  post<T>(
    path: string,
    body: Record<
      string,
      string | number | boolean | object | null | undefined
    > = {},
  ): Observable<T> {
    return this.http
      .post<T>(`${this.baseUrl}${path}`, body)
      .pipe(catchError(this.handleError));
  }

  /**
   * Performs a PUT request.
   * @param path The API endpoint path
   * @param body The request body payload
   * @returns Observable of the response
   */
  put<T>(
    path: string,
    body: Record<
      string,
      string | number | boolean | object | null | undefined
    > = {},
  ): Observable<T> {
    return this.http
      .put<T>(`${this.baseUrl}${path}`, body)
      .pipe(catchError(this.handleError));
  }

  /**
   * Performs a DELETE request.
   * @param path The API endpoint path
   * @returns Observable of the response
   */
  delete<T>(path: string): Observable<T> {
    return this.http
      .delete<T>(`${this.baseUrl}${path}`)
      .pipe(catchError(this.handleError));
  }

  /**
   * Standardized error handler for HTTP requests.
   * @param error The HTTP error response
   * @returns An observable throwing a user-facing error message
   */
  private handleError(error: HttpErrorResponse) {
    let errorMessage = 'An unknown error occurred!';
    if (
      typeof ErrorEvent !== 'undefined' &&
      error.error instanceof ErrorEvent
    ) {
      // Client-side or network error
      errorMessage = `Error: ${error.error.message}`;
    } else {
      // Backend error
      errorMessage = `Error Code: ${error.status}\nMessage: ${error.message}`;
    }
    return throwError(() => new Error(errorMessage));
  }
}

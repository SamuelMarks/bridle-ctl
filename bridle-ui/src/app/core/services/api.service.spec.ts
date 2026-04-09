import { TestBed } from '@angular/core/testing';
import { HttpTestingController, provideHttpClientTesting } from '@angular/common/http/testing';
import { provideHttpClient } from '@angular/common/http';
import { ApiService } from './api.service';

describe('ApiService', () => {
  let service: ApiService;
  let httpMock: HttpTestingController;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [
        ApiService,
        provideHttpClient(),
        provideHttpClientTesting()
      ]
    });
    service = TestBed.inject(ApiService);
    httpMock = TestBed.inject(HttpTestingController);
  });

  afterEach(() => {
    httpMock.verify();
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });

  it('should perform GET request with retries', () => {
    service.get('/test').subscribe(data => {
      expect(data).toEqual({ msg: 'success' });
    });

    // First attempt fails
    const req1 = httpMock.expectOne('/api/test');
    expect(req1.request.method).toBe('GET');
    req1.flush('Error', { status: 500, statusText: 'Server Error' });

    // Second attempt fails
    const req2 = httpMock.expectOne('/api/test');
    req2.flush('Error', { status: 500, statusText: 'Server Error' });

    // Third attempt succeeds
    const req3 = httpMock.expectOne('/api/test');
    req3.flush({ msg: 'success' });
  });

  it('should handle client-side errors', () => {
    service.get('/test-client-err').subscribe({
      next: () => fail('should have failed with the client error'),
      error: (error: Error) => {
        expect(error.message).toContain('Error: Network Error');
      }
    });

    const req1 = httpMock.expectOne('/api/test-client-err');
    const errorEvent = new ErrorEvent('Network error', { message: 'Network Error' });
    req1.error(errorEvent);

    const req2 = httpMock.expectOne('/api/test-client-err');
    req2.error(errorEvent);

    const req3 = httpMock.expectOne('/api/test-client-err');
    req3.error(errorEvent);
  });

  it('should handle server-side errors', () => {
    service.post('/test-server-err').subscribe({
      next: () => fail('should have failed with the server error'),
      error: (error: Error) => {
        expect(error.message).toContain('Error Code: 500');
        expect(error.message).toContain('Server Error');
      }
    });

    const req = httpMock.expectOne('/api/test-server-err');
    expect(req.request.method).toBe('POST');
    req.flush('Server Error', { status: 500, statusText: 'Server Error' });
  });

  it('should perform POST request', () => {
    service.post('/test-post', { data: 1 }).subscribe(data => {
      expect((data as {success: boolean}).success).toBeTrue();
    });

    const req = httpMock.expectOne('/api/test-post');
    expect(req.request.method).toBe('POST');
    expect(req.request.body).toEqual({ data: 1 });
    req.flush({ success: true });
  });

  it('should perform POST request without body', () => {
    service.post('/test-post-empty').subscribe();
    const req = httpMock.expectOne('/api/test-post-empty');
    expect(req.request.method).toBe('POST');
    expect(req.request.body).toEqual({});
    req.flush({});
  });

  it('should perform PUT request', () => {
    service.put('/test-put', { data: 2 }).subscribe(data => {
      expect((data as {success: boolean}).success).toBeTrue();
    });

    const req = httpMock.expectOne('/api/test-put');
    expect(req.request.method).toBe('PUT');
    expect(req.request.body).toEqual({ data: 2 });
    req.flush({ success: true });
  });

  it('should perform PUT request without body', () => {
    service.put('/test-put-empty').subscribe();
    const req = httpMock.expectOne('/api/test-put-empty');
    expect(req.request.method).toBe('PUT');
    expect(req.request.body).toEqual({});
    req.flush({});
  });

  it('should perform DELETE request', () => {
    service.delete('/test-delete').subscribe(data => {
      expect((data as {success: boolean}).success).toBeTrue();
    });

    const req = httpMock.expectOne('/api/test-delete');
    expect(req.request.method).toBe('DELETE');
    req.flush({ success: true });
  });
});

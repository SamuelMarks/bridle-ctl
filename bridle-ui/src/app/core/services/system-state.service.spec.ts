import { TestBed } from '@angular/core/testing';
import { HttpTestingController, provideHttpClientTesting } from '@angular/common/http/testing';
import { provideHttpClient } from '@angular/common/http';
import { SystemStateService } from './system-state.service';

describe('SystemStateService', () => {
  let service: SystemStateService;
  let httpMock: HttpTestingController;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [
        provideHttpClient(),
        provideHttpClientTesting()
      ]
    });
    service = TestBed.inject(SystemStateService);
    httpMock = TestBed.inject(HttpTestingController);
  });

  afterEach(() => {
    httpMock.verify();
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });

  it('should check health and update state', () => {
    const mockHealth: any = { rest: 'UP', rpc: 'UP', agent: 'UP' };
    
    service.checkHealth().subscribe();
    expect(service.isLoading()).toBeTrue();
    
    const req = httpMock.expectOne('/api/health');
    expect(req.request.method).toBe('GET');
    req.flush(mockHealth);
    
    expect(service.health()).toEqual(mockHealth);
    expect(service.isLoading()).toBeFalse();
  });

  it('should handle health check error', () => {
    service.checkHealth().subscribe({ error: () => {} });
    expect(service.isLoading()).toBeTrue();
    
    const req1 = httpMock.expectOne('/api/health');
    req1.flush('Error', { status: 500, statusText: 'Server Error' });
    
    const req2 = httpMock.expectOne('/api/health');
    req2.flush('Error', { status: 500, statusText: 'Server Error' });
    
    const req3 = httpMock.expectOne('/api/health');
    req3.flush('Error', { status: 500, statusText: 'Server Error' });
    
    expect(service.health()).toEqual({ rest: 'DOWN', rpc: 'DOWN', agent: 'DOWN' });
    expect(service.isLoading()).toBeFalse();
  });
});

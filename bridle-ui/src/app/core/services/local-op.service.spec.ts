import { TestBed } from '@angular/core/testing';
import {
  HttpTestingController,
  provideHttpClientTesting,
} from '@angular/common/http/testing';
import { provideHttpClient } from '@angular/common/http';
import { LocalOpService } from './local-op.service';

describe('LocalOpService', () => {
  let service: LocalOpService;
  let httpMock: HttpTestingController;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [provideHttpClient(), provideHttpClientTesting()],
    });
    service = TestBed.inject(LocalOpService);
    httpMock = TestBed.inject(HttpTestingController);
  });

  afterEach(() => {
    httpMock.verify();
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });

  it('should perform audit', () => {
    const mockRes = { output: 'res' };

    service.audit('pat', ['tool'], {}).subscribe();
    expect(service.isOperating()).toBeTrue();

    const req = httpMock.expectOne('/api/local/audit');
    expect(req.request.method).toBe('POST');
    expect(req.request.body).toEqual({
      pattern: 'pat',
      tools: ['tool'],
      args: {},
    });
    req.flush(mockRes);

    expect(service.lastResult()).toEqual(mockRes);
    expect(service.isOperating()).toBeFalse();
  });

  it('should handle audit error', () => {
    service.audit('pat', [], {}).subscribe({ error: () => {} });

    const req = httpMock.expectOne('/api/local/audit');
    req.flush('Error', { status: 500, statusText: 'Server Error' });

    expect(service.isOperating()).toBeFalse();
  });

  it('should perform fix', () => {
    const mockRes = { output: 'res', modifiedFiles: ['file1'] };

    service.fix('pat', ['tool'], {}, true).subscribe();
    expect(service.isOperating()).toBeTrue();

    const req = httpMock.expectOne('/api/local/fix');
    expect(req.request.method).toBe('POST');
    expect(req.request.body).toEqual({
      pattern: 'pat',
      tools: ['tool'],
      args: {},
      dryRun: true,
    });
    req.flush(mockRes);

    expect(service.lastResult()).toEqual(mockRes);
    expect(service.isOperating()).toBeFalse();
  });

  it('should handle fix error', () => {
    service.fix('pat', [], {}, false).subscribe({ error: () => {} });

    const req = httpMock.expectOne('/api/local/fix');
    req.flush('Error', { status: 500, statusText: 'Server Error' });

    expect(service.isOperating()).toBeFalse();
  });

  it('should clear result', () => {
    service.audit('pat', [], {}).subscribe();
    const req = httpMock.expectOne('/api/local/audit');
    req.flush({ output: 'res' });
    expect(service.lastResult()).toBeTruthy();

    service.clearResult();
    expect(service.lastResult()).toBeNull();
  });
});

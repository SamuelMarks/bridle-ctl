import { TestBed } from '@angular/core/testing';
import { HttpTestingController, provideHttpClientTesting } from '@angular/common/http/testing';
import { provideHttpClient } from '@angular/common/http';
import { BatchService } from './batch.service';

describe('BatchService', () => {
  let service: BatchService;
  let httpMock: HttpTestingController;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [
        provideHttpClient(),
        provideHttpClientTesting()
      ]
    });
    service = TestBed.inject(BatchService);
    httpMock = TestBed.inject(HttpTestingController);
  });

  afterEach(() => {
    httpMock.verify();
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });

  it('should load jobs', () => {
    const mockJobs: any[] = [{ id: '1', target: '1', status: 'COMPLETED', createdAt: '' }, { id: '2', target: '1', status: 'COMPLETED', createdAt: '' }];
    
    service.loadJobs().subscribe();
    
    const req = httpMock.expectOne('/api/batch/jobs');
    expect(req.request.method).toBe('GET');
    req.flush(mockJobs);
    
    expect(service.jobs()).toEqual(mockJobs);
  });

  it('should handle load jobs error', () => {
    let receivedError: Error | undefined;
    service.loadJobs().subscribe({ error: (err) => receivedError = err });
    const req1 = httpMock.expectOne('/api/batch/jobs');
    req1.flush('Error', { status: 500, statusText: 'Server Error' });
    const req2 = httpMock.expectOne('/api/batch/jobs');
    req2.flush('Error', { status: 500, statusText: 'Server Error' });
    const req3 = httpMock.expectOne('/api/batch/jobs');
    req3.flush('Error', { status: 500, statusText: 'Server Error' });
    expect(receivedError).toBeTruthy();
  });

  it('should load a specific job', () => {
    const mockJob: any = { id: '123', target: '1', status: 'COMPLETED', createdAt: '' };
    
    service.loadJob('123').subscribe();
    
    const req = httpMock.expectOne('/api/batch/jobs/123');
    expect(req.request.method).toBe('GET');
    req.flush(mockJob);
    
    expect(service.activeJob()).toEqual(mockJob);
  });

  it('should handle load job error', () => {
    let receivedError: Error | undefined;
    service.loadJob('123').subscribe({ error: (err) => receivedError = err });
    const req1 = httpMock.expectOne('/api/batch/jobs/123');
    req1.flush('Error', { status: 500, statusText: 'Server Error' });
    const req2 = httpMock.expectOne('/api/batch/jobs/123');
    req2.flush('Error', { status: 500, statusText: 'Server Error' });
    const req3 = httpMock.expectOne('/api/batch/jobs/123');
    req3.flush('Error', { status: 500, statusText: 'Server Error' });
    expect(receivedError).toBeTruthy();
  });

  it('should create batch fix', () => {
    const mockJob: any = { id: 'new-job', target: '1', status: 'COMPLETED', createdAt: '' };
    
    service.createBatchFix('org', 'title', 'desc', 'pat', ['tool'], { arg: 1 }).subscribe();
    
    const req = httpMock.expectOne('/api/batch/fix');
    expect(req.request.method).toBe('POST');
    expect(req.request.body).toEqual({
      org: 'org',
      issue: 'title',
      description: 'desc',
      pattern: 'pat',
      tools: ['tool'],
      tool_args: { arg: 1 },
      safety_mode: true,
      max_repos: undefined,
      max_prs_per_hour: undefined
    });
    req.flush(mockJob);
    
    expect(service.jobs()[0]).toEqual(mockJob);
  });

  it('should run pipeline', () => {
    const mockJob: any = { id: 'pipe-job', target: '1', status: 'COMPLETED', createdAt: '' };
    
    service.runPipeline('yaml').subscribe();
    
    const req = httpMock.expectOne('/api/batch/run');
    expect(req.request.method).toBe('POST');
    expect(req.request.body).toEqual({
      config_path: 'yaml',
      safety_mode: true,
      max_repos: undefined,
      max_prs_per_hour: undefined
    });
    req.flush(mockJob);
    
    expect(service.jobs()[0]).toEqual(mockJob);
  });

  it('should resume job', () => {
    // Set initial jobs
    const mockJob1: any = { id: '123', status: 'PENDING', target: '1', createdAt: '' };
    const mockJob2: any = { id: '456', status: 'PENDING', target: '1', createdAt: '' };
    service.loadJobs().subscribe();
    const req0 = httpMock.expectOne('/api/batch/jobs');
    req0.flush([mockJob1, mockJob2]);

    const mockResumedJob: any = { id: '123', status: 'RUNNING', target: '1', createdAt: '' };
    service.resumeJob('123').subscribe();
    
    const req = httpMock.expectOne('/api/batch/jobs/123/resume');
    expect(req.request.method).toBe('POST');
    req.flush(mockResumedJob);
    
    expect(service.activeJob()).toEqual(mockResumedJob);
    expect(service.jobs()[0].status).toBe('RUNNING');
  });

  it('should clear active job', () => {
    service.loadJob('123').subscribe();
    const req = httpMock.expectOne('/api/batch/jobs/123');
    req.flush({ id: '123' });
    expect(service.activeJob()).toBeTruthy();

    service.clearActiveJob();
    expect(service.activeJob()).toBeNull();
  });
});

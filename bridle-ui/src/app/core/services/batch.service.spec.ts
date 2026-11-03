import { BatchJob, BatchJobStatus } from '../models/models';
import { TestBed } from '@angular/core/testing';
import {
  HttpTestingController,
  provideHttpClientTesting,
} from '@angular/common/http/testing';
import { provideHttpClient, withXhr } from '@angular/common/http';
import { BatchService } from './batch.service';

describe('BatchService', () => {
  let service: BatchService;
  let httpMock: HttpTestingController;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [provideHttpClient(withXhr()), provideHttpClientTesting()],
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
    const mockJobs: BatchJob[] = [
      {
        id: '1',
        target: '1',
        status: BatchJobStatus.COMPLETED,
        createdAt: '',
      },
      {
        id: '2',
        target: '1',
        status: BatchJobStatus.COMPLETED,
        createdAt: '',
      },
    ];

    let result: BatchJob[] = [];
    service.loadJobs().subscribe((jobs) => (result = jobs));

    const req = httpMock.expectOne('/api/batch/jobs');
    expect(req.request.method).toBe('GET');
    req.flush(mockJobs);

    expect(result).toEqual(mockJobs);
  });

  it('should handle load jobs error', () => {
    let receivedError: Error | undefined;
    service.loadJobs().subscribe({ error: (err) => (receivedError = err) });
    const req1 = httpMock.expectOne('/api/batch/jobs');
    req1.flush('Error', { status: 500, statusText: 'Server Error' });
    const req2 = httpMock.expectOne('/api/batch/jobs');
    req2.flush('Error', { status: 500, statusText: 'Server Error' });
    const req3 = httpMock.expectOne('/api/batch/jobs');
    req3.flush('Error', { status: 500, statusText: 'Server Error' });
    expect(receivedError).toBeTruthy();
  });

  it('should load a specific job', () => {
    const mockJob: BatchJob = {
      id: '123',
      target: '1',
      status: BatchJobStatus.COMPLETED,
      createdAt: '',
    };

    let result: BatchJob | undefined;
    service.loadJob('123').subscribe((job) => (result = job));

    const req = httpMock.expectOne('/api/batch/jobs/123');
    expect(req.request.method).toBe('GET');
    req.flush(mockJob);

    expect(result).toEqual(mockJob);
  });

  it('should handle load job error', () => {
    let receivedError: Error | undefined;
    service.loadJob('123').subscribe({ error: (err) => (receivedError = err) });
    const req1 = httpMock.expectOne('/api/batch/jobs/123');
    req1.flush('Error', { status: 500, statusText: 'Server Error' });
    const req2 = httpMock.expectOne('/api/batch/jobs/123');
    req2.flush('Error', { status: 500, statusText: 'Server Error' });
    const req3 = httpMock.expectOne('/api/batch/jobs/123');
    req3.flush('Error', { status: 500, statusText: 'Server Error' });
    expect(receivedError).toBeTruthy();
  });

  it('should create batch fix', () => {
    const mockJob: BatchJob = {
      id: 'new-job',
      target: '1',
      status: BatchJobStatus.COMPLETED,
      createdAt: '',
    };

    let result: BatchJob | undefined;
    service
      .createBatchFix('org', 'title', 'desc', 'pat', ['tool'], { arg: 1 })
      .subscribe((job) => (result = job));

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
      max_prs_per_hour: undefined,
    });
    req.flush(mockJob);

    expect(result).toEqual(mockJob);
  });

  it('should run pipeline', () => {
    const mockJob: BatchJob = {
      id: 'pipe-job',
      target: '1',
      status: BatchJobStatus.COMPLETED,
      createdAt: '',
    };

    let result: BatchJob | undefined;
    service.runPipeline('yaml').subscribe((job) => (result = job));

    const req = httpMock.expectOne('/api/batch/run');
    expect(req.request.method).toBe('POST');
    expect(req.request.body).toEqual({
      config_path: 'yaml',
      safety_mode: true,
      max_repos: undefined,
      max_prs_per_hour: undefined,
    });
    req.flush(mockJob);

    expect(result).toEqual(mockJob);
  });

  it('should resume job', () => {
    const mockResumedJob: BatchJob = {
      id: '123',
      status: BatchJobStatus.RUNNING,
      target: '1',
      createdAt: '',
    };

    let result: BatchJob | undefined;
    service.resumeJob('123').subscribe((job) => (result = job));

    const req = httpMock.expectOne('/api/batch/jobs/123/resume');
    expect(req.request.method).toBe('POST');
    req.flush(mockResumedJob);

    expect(result).toEqual(mockResumedJob);
  });
});

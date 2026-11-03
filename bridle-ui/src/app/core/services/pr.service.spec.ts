import { PullRequest } from '../models/models';
import { TestBed } from '@angular/core/testing';
import {
  HttpTestingController,
  provideHttpClientTesting,
} from '@angular/common/http/testing';
import { provideHttpClient, withXhr } from '@angular/common/http';
import { PrService } from './pr.service';

describe('PrService', () => {
  let service: PrService;
  let httpMock: HttpTestingController;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [provideHttpClient(withXhr()), provideHttpClientTesting()],
    });
    service = TestBed.inject(PrService);
    httpMock = TestBed.inject(HttpTestingController);
  });

  afterEach(() => {
    httpMock.verify();
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });

  it('should load prs', () => {
    const mockPrs: PullRequest[] = [
      { id: '1', title: 'pr1', repoId: 'r1', status: 'LOCAL' },
    ];

    let result: PullRequest[] = [];
    service.loadPrs('org1').subscribe((prs) => (result = prs));

    const req = httpMock.expectOne('/api/prs?orgId=org1');
    expect(req.request.method).toBe('GET');
    req.flush(mockPrs);

    expect(result).toEqual(mockPrs);
  });

  it('should handle load prs error', () => {
    let errorRecv: Error | undefined;
    service.loadPrs('org1').subscribe({
      error: (err) => {
        errorRecv = err;
      },
    });
    const req1 = httpMock.expectOne('/api/prs?orgId=org1');
    req1.flush('Error', { status: 500, statusText: 'Server Error' });
    const req2 = httpMock.expectOne('/api/prs?orgId=org1');
    req2.flush('Error', { status: 500, statusText: 'Server Error' });
    const req3 = httpMock.expectOne('/api/prs?orgId=org1');
    req3.flush('Error', { status: 500, statusText: 'Server Error' });

    expect(errorRecv).toBeTruthy();
  });

  it('should sync prs', () => {
    const mockRes = { syncedCount: 5 };

    let result: { syncedCount: number } | undefined;
    service.syncPrs('org1', 10).subscribe((res) => (result = res));

    const req = httpMock.expectOne('/api/prs/sync');
    expect(req.request.method).toBe('POST');
    expect(req.request.body).toEqual({
      org: 'org1',
      max_prs_per_hour: 10,
      fork_org: undefined,
    });
    req.flush(mockRes);

    expect(result).toEqual(mockRes);
  });
});

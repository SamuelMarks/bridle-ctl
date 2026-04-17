import { PullRequest } from '../models/models';
import { TestBed } from '@angular/core/testing';
import {
  HttpTestingController,
  provideHttpClientTesting,
} from '@angular/common/http/testing';
import { provideHttpClient } from '@angular/common/http';
import { PrService } from './pr.service';

describe('PrService', () => {
  let service: PrService;
  let httpMock: HttpTestingController;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [provideHttpClient(), provideHttpClientTesting()],
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
    const mockPrs: object[] = [
      { id: '1', title: 'pr1', repoId: 'r1', status: 'LOCAL' },
    ];

    service.loadPrs('org1').subscribe();

    const req = httpMock.expectOne('/api/prs?orgId=org1');
    expect(req.request.method).toBe('GET');
    req.flush(mockPrs);

    expect(service.prs()).toEqual(mockPrs as object as PullRequest[]);
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
    expect(service.prs()).toEqual([]);
  });

  it('should sync prs', () => {
    const mockRes = { syncedCount: 5 };

    service.syncPrs('org1', 10).subscribe();
    expect(service.isSyncing()).toBeTrue();

    const req = httpMock.expectOne('/api/prs/sync');
    expect(req.request.method).toBe('POST');
    expect(req.request.body).toEqual({ orgId: 'org1', maxRate: 10 });
    req.flush(mockRes);

    expect(service.isSyncing()).toBeFalse();
  });

  it('should handle sync error', () => {
    service.syncPrs('org1', 10).subscribe({ error: () => {} });
    expect(service.isSyncing()).toBeTrue();

    const req = httpMock.expectOne('/api/prs/sync');
    req.flush('Error', { status: 500, statusText: 'Server Error' });

    expect(service.isSyncing()).toBeFalse();
  });
});

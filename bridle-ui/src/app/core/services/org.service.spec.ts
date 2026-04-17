import { Organization, Repository } from '../models/models';
import { TestBed } from '@angular/core/testing';
import {
  HttpTestingController,
  provideHttpClientTesting,
} from '@angular/common/http/testing';
import { provideHttpClient } from '@angular/common/http';
import { OrgService } from './org.service';

describe('OrgService', () => {
  let service: OrgService;
  let httpMock: HttpTestingController;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [provideHttpClient(), provideHttpClientTesting()],
    });
    service = TestBed.inject(OrgService);
    httpMock = TestBed.inject(HttpTestingController);
  });

  afterEach(() => {
    httpMock.verify();
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });

  it('should load orgs', () => {
    const mockOrgs: object[] = [{ id: '1', name: 'org1', provider: 'github' }];

    service.loadOrgs().subscribe();
    expect(service.isLoading()).toBeTrue();

    const req = httpMock.expectOne('/api/orgs');
    expect(req.request.method).toBe('GET');
    req.flush(mockOrgs);

    expect(service.orgs()).toEqual(mockOrgs as object as Organization[]);
    expect(service.isLoading()).toBeFalse();
  });

  it('should handle load orgs error', () => {
    service.loadOrgs().subscribe({ error: () => {} });

    // First attempt
    const req1 = httpMock.expectOne('/api/orgs');
    req1.flush('Error', { status: 500, statusText: 'Server Error' });

    // Retry 1
    const req2 = httpMock.expectOne('/api/orgs');
    req2.flush('Error', { status: 500, statusText: 'Server Error' });

    // Retry 2
    const req3 = httpMock.expectOne('/api/orgs');
    req3.flush('Error', { status: 500, statusText: 'Server Error' });

    expect(service.isLoading()).toBeFalse();
  });

  it('should ingest org', () => {
    const mockOrg: object = { id: 'new', name: 'org2', provider: 'gh' };

    service.ingestOrg('org2', 'gh', 'dburl').subscribe();
    expect(service.isLoading()).toBeTrue();

    const req = httpMock.expectOne('/api/orgs/ingest');
    expect(req.request.method).toBe('POST');
    expect(req.request.body).toEqual({
      name: 'org2',
      provider: 'gh',
      dbUrl: 'dburl',
    });
    req.flush(mockOrg);

    expect(service.orgs()).toEqual([mockOrg] as object as Organization[]);
    expect(service.isLoading()).toBeFalse();
  });

  it('should handle ingest org error', () => {
    service.ingestOrg('org2', 'gh', 'dburl').subscribe({ error: () => {} });

    const req = httpMock.expectOne('/api/orgs/ingest');
    req.flush('Error', { status: 500, statusText: 'Server Error' });

    expect(service.isLoading()).toBeFalse();
  });

  it('should load repos', () => {
    const mockRepos: object[] = [{ id: 'r1', name: 'repo1', orgId: '1' }];

    service.loadRepos('org1').subscribe();
    expect(service.isLoading()).toBeTrue();

    const req = httpMock.expectOne('/api/orgs/org1/repos');
    expect(req.request.method).toBe('GET');
    req.flush(mockRepos);

    expect(service.repos()).toEqual(mockRepos as object as Repository[]);
    expect(service.isLoading()).toBeFalse();
  });

  it('should handle load repos error', () => {
    service.loadRepos('org1').subscribe({ error: () => {} });

    const req1 = httpMock.expectOne('/api/orgs/org1/repos');
    req1.flush('Error', { status: 500, statusText: 'Server Error' });

    const req2 = httpMock.expectOne('/api/orgs/org1/repos');
    req2.flush('Error', { status: 500, statusText: 'Server Error' });

    const req3 = httpMock.expectOne('/api/orgs/org1/repos');
    req3.flush('Error', { status: 500, statusText: 'Server Error' });

    expect(service.isLoading()).toBeFalse();
  });
});

import { ComponentFixture, TestBed } from '@angular/core/testing';
import { WritableSignal, signal } from '@angular/core';
import { By } from '@angular/platform-browser';
import { of, throwError } from 'rxjs';
import { OrgsPageComponent } from './orgs-page.component';
import { OrgService } from '../../../core/services/org.service';
import { NotificationService } from '../../../core/services/notification.service';
import { Organization, Repository } from '../../../core/models/models';

interface MockOrgService {
  orgs: WritableSignal<Organization[]>;
  repos: WritableSignal<Repository[]>;
  isLoading: WritableSignal<boolean>;
  loadOrgs: jasmine.Spy;
  ingestOrg: jasmine.Spy;
  loadRepos: jasmine.Spy;
}

interface MockNotificationService {
  success: jasmine.Spy;
  error: jasmine.Spy;
  info: jasmine.Spy;
}

describe('OrgsPageComponent', () => {
  let component: OrgsPageComponent;
  let fixture: ComponentFixture<OrgsPageComponent>;
  let mockOrgService: MockOrgService;
  let mockNotificationService: MockNotificationService;

  beforeEach(async () => {
    mockOrgService = {
      orgs: signal([{ id: '1', name: 'Test Org', provider: 'github' }]),
      repos: signal([{ id: '1', name: 'Test Repo', orgId: '1' }]),
      isLoading: signal(false),
      loadOrgs: jasmine.createSpy('loadOrgs').and.returnValue(of([])),
      ingestOrg: jasmine.createSpy('ingestOrg').and.returnValue(of({})),
      loadRepos: jasmine.createSpy('loadRepos').and.returnValue(of([])),
    };

    mockNotificationService = {
      success: jasmine.createSpy('success'),
      error: jasmine.createSpy('error'),
      info: jasmine.createSpy('info'),
    };

    await TestBed.configureTestingModule({
      imports: [OrgsPageComponent],
      providers: [
        { provide: OrgService, useValue: mockOrgService },
        { provide: NotificationService, useValue: mockNotificationService },
      ],
    }).compileComponents();

    fixture = TestBed.createComponent(OrgsPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should load orgs on init', () => {
    expect(mockOrgService.loadOrgs).toHaveBeenCalled();
  });

  it('should handle load orgs error', () => {
    mockOrgService.loadOrgs.and.returnValue(
      throwError(() => new Error('error')),
    );
    component.ngOnInit();
    expect(mockNotificationService.error).toHaveBeenCalledWith(
      'Failed to load organizations',
    );
  });

  it('should ingest org and show success notification', () => {
    component.onIngestOrg({ name: 'test', provider: 'github', dbUrl: 'url' });
    expect(mockOrgService.ingestOrg).toHaveBeenCalledWith(
      'test',
      'github',
      'url',
    );
    expect(mockNotificationService.success).toHaveBeenCalledWith(
      'Organization test ingested successfully',
    );
  });

  it('should handle ingest org error', () => {
    mockOrgService.ingestOrg.and.returnValue(
      throwError(() => new Error('error')),
    );
    component.onIngestOrg({ name: 'test', provider: 'github', dbUrl: 'url' });
    expect(mockNotificationService.error).toHaveBeenCalledWith(
      'Failed to ingest organization test',
    );
  });

  it('should load repos when org is selected', () => {
    component.onSelectOrg('1');
    expect(component.selectedOrgId()).toBe('1');
    expect(mockOrgService.loadRepos).toHaveBeenCalledWith('1');
  });

  it('should handle load repos error', () => {
    mockOrgService.loadRepos.and.returnValue(
      throwError(() => new Error('error')),
    );
    component.onSelectOrg('1');
    expect(mockNotificationService.error).toHaveBeenCalledWith(
      'Failed to load repositories',
    );
  });

  it('should return correct org name from computed signal', () => {
    expect(component.selectedOrgName()).toBe('');

    component.selectedOrgId.set('1');
    expect(component.selectedOrgName()).toBe('Test Org');

    component.selectedOrgId.set('99');
    expect(component.selectedOrgName()).toBe('');
  });

  it('should render repo list when org is selected', () => {
    expect(fixture.debugElement.query(By.css('app-repo-list'))).toBeNull();

    component.selectedOrgId.set('1');
    fixture.detectChanges();

    const repoList = fixture.debugElement.query(By.css('app-repo-list'));
    expect(repoList).toBeTruthy();
  });
});

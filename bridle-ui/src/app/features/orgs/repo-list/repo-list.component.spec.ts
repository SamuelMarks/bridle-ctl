import { ComponentFixture, TestBed } from '@angular/core/testing';
import { Component } from '@angular/core';
import { RepoListComponent } from './repo-list.component';
import { By } from '@angular/platform-browser';

@Component({
  template: `<app-repo-list [orgName]="orgName" [repos]="repos"></app-repo-list>`,
  imports: [RepoListComponent]
})
class TestHostComponent {
  orgName = 'TestOrg';
  repos = [
    { id: '1', orgId: '1', name: 'repo-1', description: 'desc 1', url: 'http://link' }
  ];
}

describe('RepoListComponent', () => {
  let component: TestHostComponent;
  let fixture: ComponentFixture<TestHostComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [TestHostComponent, RepoListComponent]
    }).compileComponents();

    fixture = TestBed.createComponent(TestHostComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should render title with org name', () => {
    const title = fixture.debugElement.query(By.css('.Box-title')).nativeElement;
    expect(title.textContent).toContain('TestOrg');
  });

  it('should render repo details', () => {
    const trs = fixture.debugElement.queryAll(By.css('tbody tr'));
    expect(trs.length).toBe(1);
    
    expect(trs[0].nativeElement.textContent).toContain('repo-1');
    expect(trs[0].nativeElement.textContent).toContain('desc 1');
    
    const link = trs[0].query(By.css('a')).nativeElement;
    expect(link.href).toBe('http://link/');
  });
});
